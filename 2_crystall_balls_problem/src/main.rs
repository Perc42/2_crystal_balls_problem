fn main() {
    let ar= [1,2,4,3,8,5,9,6];
    let x=ar.len() as i32;
    let jmpamt= (x as f64).sqrt() as i32+1;
    let mut i = jmpamt; 
    while i < x {
        if ar[i as usize] == 1{
            break;
        }
        i += jmpamt;
    }
    i -= jmpamt;
    let mut j =0;
    while j< jmpamt && i<x {
        if ar[i as usize]==1{
            print!("{}",i);
            break;
        }
        i+=1;
        j+=1;
    }
}
