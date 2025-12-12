use super::model::{Present, TreeRegion};
use hashbrown::HashMap;

pub fn solve_part_one(presents: &[Present], regions: &[TreeRegion]) -> String {
    let present_index: HashMap<usize, &Present> = presents.iter().map(|p| (p.index, p)).collect();
    let mut result: usize = 0;

    for region in regions.iter() {
        result += process_region(&present_index, region)
    }

    result.to_string()
}

fn process_region(present_index: &HashMap<usize, &Present>, region: &TreeRegion) -> usize {
    let region_area = region.area();
    let number_present = region.total_quantity();

    let number_present_slots = region_area / Present::TOTAL_AREA;

    let actual_total_present = region
        .present_quantity
        .iter()
        .enumerate()
        .map(|(index, count)| present_index[&index].actual_area() * count)
        .sum::<usize>();

    // Easy filters

    // Not enough space
    if region_area < actual_total_present {
        return 0;
    }

    // Way enough place
    if number_present_slots >= number_present {
        return 1;
    }

    // Complex filters

    unreachable!("Thank you Eric, I love you")
}
