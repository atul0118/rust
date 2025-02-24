use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env;

const SILVER:u8   = 0;
const GOLD:u8     = 1;
const TITANIUM:u8 = 2;
const PRIME:u8    = 3;

/*
 * Idea is to read freq-cost values from 3 files corresponding to
 * each cluster and store them in a map first. Then create a min-heap
 * out of the map. Map is needed as same frequency can be available for
 * multiple clusters.
 * Final data can then be populated by just traversing the heap.
 */

type HeapType = Reverse<(u32,Vec<u32>)>;

fn read_cluster_data(cluster:u8, fname: &String,
                    freq_cost_map:&mut HashMap<u32, Vec<u32>>) -> ()
{
    /*
     * Reading frequency and cost of "cluster"
     */
    let mut fp = File::open(fname).unwrap();
    let mut file_data = String::new();
    fp.read_to_string(&mut file_data).expect("Failed to read data");

    for line in file_data.lines() {
        let data:Vec<&str> = line.split(',').collect();
        let freq:u32 = data[0].parse().unwrap();
        let cost:u32 = data[1].parse().unwrap();

        // The key to map is freq. If it is not present, insert the freq with value as 4 0's.
        // I'll handle 0s while writing final data to file.
        let value = freq_cost_map.get_mut(&freq);
        match value {
            Some(vec) => {
                vec[cluster as usize] = cost;
            },
            None => {
                freq_cost_map.insert(freq, vec![0; 4]); // note this order of value;size is
                                                              // opposite of stl order
                freq_cost_map.get_mut(&freq).unwrap()[cluster as usize] = cost;
            }
        };

    }

}

fn main()
{
    let cmdline_args:Vec<String> = env::args().collect();
    let mut freq_cost_heap:BinaryHeap<HeapType> = BinaryHeap::new();
    let mut freq_cost_map:HashMap<u32,Vec<u32>> = HashMap::new();
    let mut output_file;

    for (idx,arg) in cmdline_args.iter().enumerate() {
        println!("arg{}: {}", idx, arg);
    }

    if cmdline_args.len() < 4 {
        println!("REQUIRED: cost files for 3 clusters required");
        std::process::exit(1);
    }

    let fname_silver = &cmdline_args[1];
    let fname_gold = &cmdline_args[2];
    let fname_prime = &cmdline_args[3];

    read_cluster_data(SILVER, fname_silver, &mut freq_cost_map);
    read_cluster_data(GOLD, fname_gold, &mut freq_cost_map);
    read_cluster_data(PRIME, fname_prime, &mut freq_cost_map);

    // Finally push the data to heap, with first value of heap as frequency
    for (key, val) in freq_cost_map.iter() {

        freq_cost_heap.push(Reverse((*key, val.to_vec()))); // understand why dereferencing is
                                                            // incorrect here

    }

    println!("Total unique frequencies: {}", freq_cost_heap.len());
    output_file = File::create("out.csv").unwrap();
    output_file.write_all("frequency,cost_silver,cost_gold,cost_titanium,cost_prime\n".to_string().as_bytes()).expect("Failed to write in csv format");

    while freq_cost_heap.len() != 0 {
        let entry = freq_cost_heap.pop();
        match entry {
            Some(Reverse((freq, values_vec))) => {
                println!("freq:{} cost:{:?}", freq, values_vec);
                // convert 0 to "" before writing to file
                let data_as_string:Vec<String> = values_vec.iter().map(|x| {
                    if *x == 0 {
                        "".to_string()
                    } else {
                        x.to_string()
                    }
                }).collect();
                write!(output_file, "{},{},{},{},{}\n", freq, data_as_string[0], data_as_string[1],
                    data_as_string[2], data_as_string[3]).expect("Failed to write in csv format");
            },
            None => {}
        };
    }

}
