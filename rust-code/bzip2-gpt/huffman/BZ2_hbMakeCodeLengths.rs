use crate::BZ2_bz__AssertH__fail;

fn weight_of(zz0: i32) -> i32 {
    zz0 & 0xffffff00
}

fn depth_of(zz1: i32) -> i32 {
    zz1 & 0x000000ff
}

fn my_max(zz2: i32, zz3: i32) -> i32 {
    if zz2 > zz3 { zz2 } else { zz3 }
}

fn add_weights(zw1: i32, zw2: i32) -> i32 {
    (weight_of(zw1) + weight_of(zw2)) | (1 + my_max(depth_of(zw1), depth_of(zw2)))
}

fn up_heap(z: i32, heap: &mut [i32], weight: &[i32]) {
    let mut zz = z;
    let tmp = heap[zz as usize];
    while weight[tmp as usize] < weight[heap[(zz >> 1) as usize] as usize] {
        heap[zz as usize] = heap[(zz >> 1) as usize];
        zz >>= 1;
    }
    heap[zz as usize] = tmp;
}

fn down_heap(z: i32, heap: &mut [i32], weight: &[i32], n_heap: i32) {
    let mut zz = z;
    let mut yy;
    let tmp = heap[zz as usize];
    loop {
        yy = zz << 1;
        if yy > n_heap {
            break;
        }
        if yy < n_heap && weight[heap[(yy + 1) as usize] as usize] < weight[heap[yy as usize] as usize] {
            yy += 1;
        }
        if weight[tmp as usize] < weight[heap[yy as usize] as usize] {
            break;
        }
        heap[zz as usize] = heap[yy as usize];
        zz = yy;
    }
    heap[zz as usize] = tmp;
}

pub fn bz2_hb_make_code_lengths(
    len: &mut [u8],     // Equivalent to UChar*
    freq: &[i32],       // Equivalent to Int32*
    alpha_size: i32,
    max_len: i32,
) {
    let mut n_nodes: i32;
    let mut n_heap: i32;
    let mut n1: i32;
    let mut n2: i32;
    let mut too_long: bool;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;

    let mut heap = [0i32; 258 + 2];
    let mut weight = [0i32; 258 * 2];
    let mut parent = [0i32; 258 * 2];

    // Initialize the weight array
    for i in 0..alpha_size {
        weight[(i + 1) as usize] = if freq[i as usize] == 0 { 1 } else { freq[i as usize] } << 8;
    }

    loop {
        n_nodes = alpha_size;
        n_heap = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        // Insert elements into the heap
        for i in 1..=alpha_size {
            parent[i as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = i;
            up_heap(n_heap, &mut heap, &weight);
        }

        // Assert that heap is within bounds
        if n_heap >= 258 + 2 {
            BZ2_bz__AssertH__fail(2001);
        }

        // Perform heap operations to create the Huffman tree
        while n_heap > 1 {
            n1 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            down_heap(1, &mut heap, &weight, n_heap);

            n2 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            down_heap(1, &mut heap, &weight, n_heap);

            n_nodes += 1;
            parent[n1 as usize] = n_nodes;
            parent[n2 as usize] = n_nodes;
            weight[n_nodes as usize] = add_weights(weight[n1 as usize], weight[n2 as usize]);
            parent[n_nodes as usize] = -1;

            n_heap += 1;
            heap[n_heap as usize] = n_nodes;
            up_heap(n_heap, &mut heap, &weight);
        }

        // Assert that n_nodes is within bounds
        if n_nodes >= 258 * 2 {
            BZ2_bz__AssertH__fail(2002);
        }

        too_long = false;
        for i in 1..=alpha_size {
            j = 0;
            k = i;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            len[(i - 1) as usize] = j as u8;
            if j > max_len {
                too_long = true;
            }
        }

        if !too_long {
            break;
        }

        // Reduce weights to handle too-long codes
        for i in 1..=alpha_size {
            j = weight[i as usize] >> 8;
            j = 1 + (j / 2);
            weight[i as usize] = j << 8;
        }
    }
}
