/*
    calculate XOR distance between a nodeID and out nodeID
    store nodeIDs in such a manner that the number of 0 bits in distance between us corrosponds to the index of the bucket
        for example:
            the furthest distance from us thier XOR distance should be the maximum value of the 160bit field
            example1:
                us : 0 0 0 0 ... 0 0
                them : 1 1 1 1 ... 1 1
                distance : 1 1 1 1 ... 1 1 ---> put in the last bucket
            example2:
                us : 0 0 0 0 ... 0 0
                them : 0 0 0 0 ... 0 1
                distance : 0 0 0 0 ... 0 1 ---> put in the first bucket

| Step | Description                                                        |
| ---- | ------------------------------------------------------------------ |
| 1    | Compute XOR distance between local node and new node               |
| 2    | Identify appropriate bucket based on distance                      |
| 3    | If bucket has space, add node to end                               |
| 4    | If full, ping oldest; evict if unresponsive, else discard new node |
| 5    | If node already exists, move it to end of list                     |



1. Adding a Node to a Bucket
Step-by-step:
    1. Determine the Distance:
        Calculate the XOR distance between the new node's ID and your own Node ID.
        Use this to find the appropriate bucket. For example, if the distance has its first set bit at position i, place the node in bucket i.
    2. Check Bucket Size:
        If the bucket is not full (i.e. it has fewer than k nodes), simply add the new node to the end of the bucket.
    3. If the Bucket is Full:
            Kademlia specifies that each bucket is a least-recently seen list (oldest at the front, newest at the end).
            Ping the oldest node in the bucket (i.e., the first entry).
                If it responds: do not add the new node.
                If it does not respond: evict it and add the new node to the end of the bucket.
        ✅ This favors long-lived, stable nodes over new or potentially unstable ones.
    4. If the Node Already Exists:
        Move it to the end of the bucket to mark it as recently seen.
2. Bucket Splitting (in some implementations)
    In vanilla Kademlia, when the bucket covering the own Node ID's range is full, it is split into two smaller buckets to allow for finer resolution near the node.
    BitTorrent DHT typically doesn't do recursive splitting like original Kademlia — it uses a non-recursive routing table to save memory and simplify behavior.
*/

/*
    when spliting table
        split only if:
            the buvket is full, and is not the at last index (159) which should contain our node

    splitting:
        take all nodes from inside last bucket
        and add a new node  (ore remove last bucket, add 2 and redistrubute the nodes)
        redistrubute the nodes of the old last bucket

    ? IMPORTANT: new nodes are pushed to the start of the vec not the end
    ?   se we start pinging questionable nodes from the start of the vec which are the newest
    ?   so we can keep the old good nodes which are more likely to be stable
*/

use crate::{
    dht::{
        bucket::{Bucket, MAX_BUCKETS},
        node::{Node, NodeId},
    },
    log::{error, info, warning},
    utils::{count_leading_zeros, xor_distance},
};

#[derive(Debug)]
pub struct RoutingTable {
    pub buckets: Vec<Bucket>,
    my_node_id: NodeId,
}

impl RoutingTable {
    pub fn new(my_node_id: NodeId) -> Self {
        RoutingTable {
            buckets: vec![Bucket::new()],
            my_node_id,
        }
    }

    pub fn add(&mut self, node: Node) -> Result<(), String> {
        //! not sure when we will need it
        if node.id == self.my_node_id {
            error("got our own node not sure if this will ever happen".to_string());
            return Err(String::from(
                "got our own node not sure if this will ever happen",
            ));
        }

        let distance = xor_distance(node.id.0, self.my_node_id.0);
        // the unwrap shouldnt cause any errors
        let leading_zeros = count_leading_zeros(distance.try_into().unwrap()) as usize;

        // means distance is 0 so our own node
        if leading_zeros == MAX_BUCKETS {
            error("the used node is the same as ours because the id is the same".to_string());
            return Err("the used node is the same as ours because the id is the same".to_string());
        }
        match self.add_to_bucket(node, leading_zeros) {
            Ok(_) => Ok(()),
            Err(e) => {
                error(format!("failed to add the node: {}", e));
                Err("".to_string())
            }
        }
    }

    pub fn add_to_bucket(&mut self, node: Node, leading_zeros: usize) -> Result<(), String> {
        let bucket_index = if leading_zeros >= self.buckets.len() {
            self.buckets.len() - 1
        } else {
            leading_zeros
        };

        // * split the buckets only when we are in the space of our own nodeID
        if !self.buckets[bucket_index].add_node(node.clone()) {
            warning(format!("spliting the bucket {}", bucket_index));

            //? bucket_index != MAX_BUCKETS - 1: means we need to be in our own node bucket space to be able to split bucket
            //? should split if the last bucket(which is our node space) is full
            let can_split =
                bucket_index == self.buckets.len() - 1 && bucket_index != MAX_BUCKETS - 1;

            warning(format!("can it split?: {}", can_split));

            if can_split {
                let split_bucket = match self.buckets.pop() {
                    Some(bucket) => bucket,
                    None => panic!("no buckets present in the RoutingTable"),
                };
                warning(format!("split_bucket: {:?}", split_bucket));

                self.buckets.push(Bucket::new());
                self.buckets.push(Bucket::new());

                warning(format!(
                    "split the bucket {}, new number of buckets: {}",
                    bucket_index,
                    self.buckets.len()
                ));

                // after splitting the buckets, redustribute the nodes in trhe split node
                // rev to add the oldest nodes first (not sure if its correct)
                for n in split_bucket.nodes.into_iter().rev() {
                    self.add(n).unwrap();
                }

                // after failing to add the node and succeding to split nodes we try to inser again
                self.add_to_bucket(node, leading_zeros).map_err(|e| {
                    format!(
                        "failed to add node to routing table after splitting the last bucket: {}",
                        e
                    )
                })?;
            }
        } else {
            info(format!(
                "node: {:?} added to bucket: {}, leading zeros: {}",
                node, bucket_index, leading_zeros
            ));

            return Ok(());
        };

        // node is not added but that is ok
        Ok(())
    }
}
