use rand::Rng;

fn main() {
    let test = vec!(3, 1, 2, 0, 4, 6, 8, 5, 7);
    println!("{:?}", QuickSort(test));
}

fn QuickSort(mass: Vec<i32>) -> Vec<i32>{
    let randNum: usize;
    if (mass.len() <= 1) {
        return mass;
    } else {
        randNum = rand::thread_rng().gen_range(1..mass.len());
    }

    let mut l_nums= Vec::new();
    let mut e_nums= vec!(mass[randNum]);
    let mut b_nums= Vec::new();
    for i in mass.iter(){
        if (i < &mass[randNum]){ l_nums.push(i.clone()); continue;}
        if (i > &mass[randNum]){ b_nums.push(i.clone()); }
    }

    return ConverOneVec(&[QuickSort(l_nums), e_nums, QuickSort(b_nums)]);
}

fn ConverOneVec(vectors: &[Vec<i32>]) -> Vec<i32>{
    let mut returElemAdd: Vec<i32> = Vec::new();
    for i in 0..vectors.len(){
        let mut elem = vectors[i].clone();
        returElemAdd.append(&mut elem);
    }
    return returElemAdd.clone();
}
