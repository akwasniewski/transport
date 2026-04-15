use ordered_float::OrderedFloat;
use crate::{graph::Graph, index_vec};
use crate::utility::IndexVec;
// do I need to write it mysefl? No. Do I want to? Yes.

fn kth<T: Ord>(indices: &mut [usize], data: &[T], k:usize) -> usize
{
    // insertion is optimal for small arrays
    for i in 1..indices.len(){
        let mut j=i;
        while j>0 && data[indices[j-1]]>data[indices[j]]{
            indices.swap(j,j-1);
            j-=1;
        }
    }
    indices[k]
}

fn lomuto_partition<T: Ord>(indices: &mut [usize], pivot: usize, data: &[T]) -> usize {
    let last = indices.len() - 1;
    let pivot_idx = indices.iter().position(|&x| x == pivot).unwrap();
    indices.swap(pivot_idx, last);

    let mut i = 0;
    for j in 0..last {                
        if data[indices[j]] < data[pivot] {
            indices.swap(i, j);
            i += 1;
        }
    }
    indices.swap(i, last);   
    i
}

fn quickselect<T: Ord>(indices: &mut[usize], data: &[T],  k: usize) -> usize{
    if indices.len()<=5{
        return kth(indices, data, k);
    }
    let mut medians_indices: Vec<usize> = Vec::new();
    for i in (0..indices.len()).step_by(5){
        let end = (i+5).min(indices.len());
       medians_indices.push(kth(&mut indices[i..end], data,(end-i)/2));

    }
    let median_indices_len = medians_indices.len();
    let pivot = quickselect(&mut medians_indices,data, median_indices_len/2);
    let pivot_position = lomuto_partition(indices, pivot, data);
    if pivot_position==k{
        return pivot;
    }
    else if pivot_position>k{
        return quickselect(&mut indices[0..pivot_position],data, k);
    }
    quickselect(&mut indices[pivot_position+1..], data,k-pivot_position-1)
}



impl Graph{
    // division_depth is log of the number of regions generated
    pub fn divide_into_regions(&mut self, division_depth: u32){
        let latitudes: Vec<OrderedFloat<f32>> = self.vertices.iter().map(|v| OrderedFloat(v.coords.0)).collect();
        let longitudes: Vec<OrderedFloat<f32>> = self.vertices.iter().map(|v| OrderedFloat(v.coords.1)).collect();
        let mut indices: Vec<usize> = (0..self.vertices.len()).collect();

        self.regions = Some(index_vec![0;self.vertices.len()]);
        let mut region_counter = 0;
        
        self.kd_division(&mut indices, division_depth, &latitudes, &longitudes, &mut region_counter);

    }

    fn kd_division(&mut self, indices: &mut[usize], 
        division_depth: u32, 
        latitudes: &[OrderedFloat<f32>], 
        longitudes: &[OrderedFloat<f32>], 
        region_counter: &mut u32)
    {
        if division_depth==0{
            let regions = self.regions.as_mut().unwrap();
           for &cur in indices.iter(){
                regions[cur as u32] = *region_counter;
            }
            *region_counter+=1;
            return;
        }
        let data = match division_depth.is_multiple_of(2){
            true => latitudes,
            false => longitudes
        };

        let pivot = quickselect(indices,data,  indices.len()/2);
        let pivot_position = lomuto_partition(indices, pivot, data);

        self.kd_division(&mut indices[..pivot_position], division_depth-1, latitudes, longitudes, region_counter);
        self.kd_division(&mut indices[pivot_position..], division_depth-1, latitudes, longitudes, region_counter);
    }
}
