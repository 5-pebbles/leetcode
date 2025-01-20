/// A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.
///
/// The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:
///
/// - lefti is the x coordinate of the left edge of the ith building.
/// - righti is the x coordinate of the right edge of the ith building.
/// - heighti is the height of the ith building.
///
/// You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
///
/// The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate 0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.
///
///
/// Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]
///
/// # Example
/// ```
/// use the_skyline_problem::get_skyline;
///
/// let buildings = vec![vec![0, 2, 3], vec![2, 5, 3]];
/// assert_eq!(get_skyline(buildings), [[0,3],[5,0]]);
///
/// let buildings = vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]];
/// assert_eq!(get_skyline(buildings), [[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]]);
/// ```
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // This is just a sweep line. It's slow, but I am going to bed.
    let mut lines: Vec<i32> = Vec::new();

    for building in buildings.iter() {
        lines.push(building[0]);
        lines.push(building[1]);
    }
    lines.sort();

    let mut skyline: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let max_height = buildings
            .iter()
            .filter(|building| building[0] <= line && building[1] > line)
            .max_by_key(|x| x[2])
            .map(|x| x[2])
            .unwrap_or_default();

        if max_height == skyline.last().map(|x| x[1]).unwrap_or_default() {
            continue;
        }

        skyline.push(vec![line, max_height]); // [x, y]
    }

    skyline
}
