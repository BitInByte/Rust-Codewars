fn main() {
    // let signature = [1.0, 1.0, 1.0];
    let signature = [10.959281490978432, 2.9330764987334135, 13.25686969028595];
    let total_sequence = 2;
    let result_tribonacci_sequence = tribonacci(&signature, total_sequence);
    println!("Result: {:?}", result_tribonacci_sequence);
}

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    match check_number_irregularity(n, &signature) {
        Ok(_) => (),
        Err(error) => return error,
    };

    let mut tribonacci_sequence = convert_list_f64_to_vector(signature);

    calculate_tribonacci_sequence(&mut tribonacci_sequence, n).to_vec()
}

fn check_number_irregularity(number: usize, signature: &[f64; 3]) -> Result<(), Vec<f64>> {
    if number == 0 {
        return Err(Vec::new());
    } else if number < 3 {
        let tribonacci_sequence = build_vector_based_on_number_elements(number, signature);
        return Err(tribonacci_sequence);
    }

    return Ok(());
}

fn build_vector_based_on_number_elements(number: usize, signature: &[f64]) -> Vec<f64> {
    let mut tribonacci_sequence: Vec<f64> = Vec::new();
    for i in 0..number {
        let current_index_value = signature[i];
        tribonacci_sequence.push(current_index_value);
    }
    tribonacci_sequence
}

fn convert_list_f64_to_vector(list: &[f64]) -> Vec<f64> {
    list.to_vec()
}

fn calculate_tribonacci_sequence(tribonacci_sequence: &mut Vec<f64>, n: usize) -> Vec<f64> {
    for i in 3..n {
        let next_number = calculate_next_sequence_number(&tribonacci_sequence, i);
        tribonacci_sequence.push(next_number);
    }

    tribonacci_sequence.to_vec()
}

fn calculate_next_sequence_number(tribonacci_sequence: &Vec<f64>, current_index: usize) -> f64 {
    let mut next_number = 0.0;
    for j in 1..4 {
        let current_iteration_index = current_index - j;
        next_number += tribonacci_sequence[current_iteration_index];
    }

    next_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            tribonacci(&[0., 1., 1.], 10),
            vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
        );
        assert_eq!(
            tribonacci(&[1., 0., 0.], 10),
            vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]
        );
        assert_eq!(
            tribonacci(&[0., 0., 0.], 10),
            vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]
        );
        assert_eq!(
            tribonacci(&[1., 2., 3.], 10),
            vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]
        );
        assert_eq!(
            tribonacci(&[3., 2., 1.], 10),
            vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]
        );
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(
            tribonacci(&[0.5, 0.5, 0.5], 30),
            vec![
                0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5,
                1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5,
                266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5
            ]
        );
    }
}
