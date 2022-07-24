fn main() {
    let mut test = vec!(3, 1, 2, 0, 0, 4, 6, 8, 5, 7);
    quick_sort(test.as_mut_slice());
    println!("{:?}", test);
}

fn quick_sort(sort_mass: &mut [i32]){
    if sort_mass.len() < 2 {
        return;
    }

    let mut e_min:Vec<i32> = Vec::new();
    let mut e_max:Vec<i32> = Vec::new();

    for i in &sort_mass[1..]{
        if *i < sort_mass[0]{
            e_min.push(*i);
        }
        if *i > sort_mass[0]{
            e_max.push(*i);
        }
    }
    quick_sort(e_min.as_mut_slice());
    quick_sort(e_max.as_mut_slice());

    e_min.push(sort_mass[0]);
    e_min.extend(e_max);

    for (num, i) in e_min.iter().enumerate()
	{
		sort_mass[num]=*i;
	}
}
