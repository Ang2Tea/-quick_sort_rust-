use rand::Rng;

fn main() {
}

fn QuickSort(mass: Vec<i32>) -> Vec<i32>{
    let randNum: i32;
    if (mass.len() <= 1) {
        return mass;
    } else {
        randNum = rand::thread_rng().gen_range(1..mass.len());
        println!("{:?}", randNum);
    }

    let mut l_nums  = Vec::from(mass[randNum..mass.len()]);
    let mut e_nums = vec![randNum; mass.count(randNum)];
    let mut b_nums= Vec::from(mass[0..randNum]);

    let mut returElemAdd: Vec<i32>;
    returElemAdd.append(&mut l_nums);
    returElemAdd.append(&mut e_nums);
    returElemAdd.append(&mut b_nums);

    let returElem= returElemAdd.clone();
    return returElem;
}
