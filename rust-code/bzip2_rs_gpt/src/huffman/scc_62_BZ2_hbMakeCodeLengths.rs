

pub fn bz2_hb_make_code_lengths(len: &mut [u8], freq: &[i32], alpha_size: i32, max_len: i32) {
    // 变量定义
    let mut n_nodes;
    let mut n_heap;
    let mut n1;
    let mut n2;
    let mut i: i32;
    let mut j: i32;
    let mut k;
    let mut too_long: bool;

    let mut heap = vec![0; 260]; // 258 + 2
    let mut weight = vec![0; 516]; // 258 * 2
    let mut parent = vec![-1; 516]; // 258 * 2 初始化为-1


    // 初始化权重
    for i in 0..alpha_size as usize {
        weight[i + 1] = (if freq[i] == 0 { 1 } else { freq[i] }) << 8;
    }
    loop {
        n_nodes = alpha_size;
        n_heap = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        // 构建初步的堆
        for i in 1..=alpha_size as usize {
            n_heap += 1;
            heap[n_heap] = i as i32;

            // 堆排序调整
            let mut zz = n_heap;
            let tmp = heap[zz];
            while weight[tmp as usize] < weight[heap[zz >> 1] as usize] {
                heap[zz] = heap[zz >> 1];
                zz >>= 1;
            }
            heap[zz] = tmp;
        }

        // 堆验证
        assert!(n_heap < 260, "Heap size exceeded limits");

        // 创建Huffman树的逻辑，包括处理 n1, n2, 堆的调整等...
        // 创建Huffman树
    while n_heap > 1 {
        n1 = heap[1];
        // 堆排序调整
        heap[1] = heap[n_heap];
        n_heap -= 1;
        let mut zz = 1;
        let mut tmp = heap[zz];
        while {
            let mut yy  = zz << 1;
            if yy > n_heap {
                false
            } else {
                if yy < n_heap && weight[heap[yy + 1] as usize] < weight[heap[yy] as usize] {
                    yy += 1;
                }
                if weight[tmp as usize] < weight[heap[yy] as usize] {
                    false
                } else {
                    heap[zz] = heap[yy];
                    zz = yy;
                    true
                }
            }
        } {}
        heap[zz] = tmp;

        // 重复上述操作，得到第二个最小权重的节点
        n2 = heap[1];
        heap[1] = heap[n_heap];
        n_heap -= 1;
        zz = 1;
        tmp = heap[zz];
        while {
            let mut yy = zz << 1;
            if yy > n_heap {
                false
            } else {
                if yy < n_heap && weight[heap[yy + 1] as usize] < weight[heap[yy] as usize] {
                    yy += 1;
                }
                if weight[tmp as usize] < weight[heap[yy] as usize] {
                    false
                } else {
                    heap[zz] = heap[yy];
                    zz = yy;
                    true
                }
            }
        } {}
        heap[zz] = tmp;

        // 创建新的父节点
        n_nodes += 1;
        parent[n1 as usize] = n_nodes;
        parent[n2 as usize] = n_nodes;


        let weight_n1 = weight[n1 as usize];
        let weight_n2 = weight[n2 as usize];
        let combined_weight = ((weight_n1 as u32 & 0xffffff00) + (weight_n2 as u32 & 0xffffff00)) as i32;
        let depth = 1 + std::cmp::max(weight_n1 & 0x000000ff, weight_n2 & 0x000000ff);
        weight[n_nodes as usize] = combined_weight | depth;
        parent[n_nodes as usize] = -1;

        n_heap += 1;
        heap[n_heap] = n_nodes;
        zz = n_heap;
        tmp = heap[zz];
        while weight[tmp as usize] < weight[heap[zz >> 1] as usize] {
            heap[zz] = heap[zz >> 1];
            zz >>= 1;
        }
        heap[zz] = tmp;
    }


    // 检查编码长度是否过长
    too_long = false;
    for i in 1..=alpha_size as usize {
        j = 0;
        k = i;
        while parent[k] >= 0 {
            k = parent[k] as usize;
            j += 1;
        }
        len[i - 1] = j as u8;
        if j > max_len as i32 { // 修改类型转换
            too_long = true;
        }
    }

    // 如果没有超过最大编码长度，则结束循环
    if !too_long {
        break;
    }
    
    // 在计算 weight 的地方
    for i in 1..=alpha_size as usize {
        j = weight[i] >> 8;
        j = 1 + (j / 2);
        weight[i] = (j as i32) << 8; // 对 j 进行类型转换
    }
}

    assert!(n_nodes < 516, "Number of nodes exceeded limits");
}





