fn main(){
    let v = [1,2,3,4,5,6];
    let off = 7;
    println!("{:?}", v[(0+off)%v.len()]);
    println!("{:?}", v[(1+off)%v.len()]);
    println!("{:?}", v[(2+off)%v.len()]);
    println!("{:?}", v[(3+off)%v.len()]);
    println!("{:?}", v[(4+off)%v.len()]);
    println!("{:?}", v[(5+off)%v.len()]);
}
