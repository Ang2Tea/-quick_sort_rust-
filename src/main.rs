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
        if (i < &mass[randNum]){
            l_nums.push(i.clone());
        }
        else if (i > &mass[randNum]){
            b_nums.push(i.clone());
        }
    }

    let mut returElemAdd: Vec<i32> = Vec::new();
    returElemAdd.append(&mut QuickSort(l_nums));
    returElemAdd.append(&mut e_nums);
    returElemAdd.append(&mut QuickSort(b_nums));
    let returElem= returElemAdd.clone();
    return returElem;
}
