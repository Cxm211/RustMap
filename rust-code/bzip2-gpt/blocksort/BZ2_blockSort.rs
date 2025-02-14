use crate::blocksort::fallbackSort::fallback_sort;
use crate::blocksort::mainSort::main_sort;
use crate::EState;

pub fn bz2_block_sort(s: &mut EState) {
    // Extract relevant fields from the EState struct
    let ptr = s.ptr.as_mut().unwrap();
    let block = s.block.as_mut().unwrap();
    let ftab = s.ftab.as_mut().unwrap();
    let nblock = s.nblock;
    let verb = s.verbosity;
    let mut wfact = s.work_factor;
    let quadrant: &mut [u16];
    let mut budget: i32;
    let mut budget_init: i32;
    let mut i: i32;

    if nblock < 10000 {
        // Use the fallbackSort function if block size is small
        fallback_sort(s.arr1.as_mut().unwrap(), s.arr2.as_mut().unwrap(), ftab, nblock, verb);
    } else {
        // Set up quadrant
        i = nblock + (2 + 12 + 18 + 2);
        if i & 1 != 0 {
            i += 1;
        }
        // Convert `i` to `usize` for correct slice indexing
        let index = i as usize;

        // Create the quadrant slice from block
        quadrant = unsafe { std::slice::from_raw_parts_mut(block[index..].as_mut_ptr() as *mut u16, (block.len() - index) / 2) };

        // Adjust work factor
        if wfact < 1 {
            wfact = 1;
        }
        if wfact > 100 {
            wfact = 100;
        }

        // Initialize budget
        budget_init = nblock * ((wfact - 1) / 3);
        budget = budget_init;

        // Perform main sorting
        main_sort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);

        if verb >= 3 {
            println!(
                "      {} work, {} block, ratio {:5.2}",
                budget_init - budget,
                nblock,
                (budget_init - budget) as f32 / (nblock.max(1) as f32)
            );
        }

        if budget < 0 {
            if verb >= 2 {
                println!("    too repetitive; using fallback sorting algorithm");
            }
            fallback_sort(s.arr1.as_mut().unwrap(), s.arr2.as_mut().unwrap(), ftab, nblock, verb);
        }
    }

    // Set s.orig_ptr to the index where ptr[i] is 0
    s.orig_ptr = -1;
    for i in 0..s.nblock {
        if ptr[i as usize] == 0 {
            s.orig_ptr = i;
            break;
        }
    }

    // Ensure orig_ptr is set
    assert!(s.orig_ptr != -1, "BZ2_bz__AssertH__fail(1003)");
}
