/// There are `n` gas stations along a circular route, where the amount of gas at the `ith` station is `gas[i]`.
///
/// You have a car with an unlimited gas tank and it costs `cost[i]` of gas to travel from the `ith` station to its next `(i + 1)th` station. You begin the journey with an empty tank at one of the gas stations.
///
/// Given two integer arrays `gas` and `cost`, return _the starting gas station 's index if you can travel around the circuit once in the clockwise direction, otherwise return_ `-1`. If there exists a solution, it is **guaranteed** to be **unique**
///
/// # Example
/// ```
/// use gas_station::can_complete_circuit;
///
/// assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
/// assert_eq!(can_complete_circuit(vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3);
/// assert_eq!(can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
/// assert_eq!(can_complete_circuit(vec![4, 5, 2, 6, 5, 3], vec![3, 2, 7, 3, 2, 9]), -1);
/// ```
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut deficit: i32 = 0;
    let (mut tank, mut station): (i32, i32) = (0, 0);

    for ((index, gas), cost) in gas.into_iter().enumerate().zip(cost.into_iter()) {
        tank += gas - cost;
        deficit += gas - cost;

        if tank < 0 {
            station = index as i32 + 1;
            tank = 0
        }
    }

    if deficit < 0 {
        return -1;
    }
    station
}
