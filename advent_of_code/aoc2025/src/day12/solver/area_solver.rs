use super::super::types::Region;

fn presents_areas(i: usize) -> u16 {
    // Calculated by visual inspection
    [7,7,7,5,7,6][i]
}

pub fn can_presents_fit_in_region(
    region: &Region,
    presents: &[u16]
) -> bool {
    let total_presents_areas = presents.iter().enumerate()
        .map(|(i, &cnt)| cnt * presents_areas(i))
        .sum::<u16>();
    let region_area = region.length * region.width;
    
    total_presents_areas <= region_area
}

pub fn can_presents_fit_in_region_coarse(
    region: &Region,
    presents: &[u16]
) -> bool {
    let total_presents_areas = 9 * presents.iter().sum::<u16>();
    let region_area = region.length * region.width;
    
    total_presents_areas <= region_area
}