//! Example of using the [`ESearch`] module
use ncbi::{EntrezDb, ESearch};

fn main() {
    let builder = ESearch::new(EntrezDb::Protein)
        .term("deaminase")
        .max(500)
        .start(1500);
    let result = builder.search();
    if let Some(result) = result {

        assert!(result.count() >= 3403057);
        assert_eq!(result.ret_max(), 500);
        assert_eq!(result.ret_start(), 1500);
        assert_eq!(result.id_list().count(), 500);

        println!("{:?}", result)
    }

    let result = ESearch::new(EntrezDb::Protein)
        .term("Enzymatic modification AND (transport OR excretion) bacteria")
        .search();

    if let Some(result) = result {
        assert!(result.count() >= 31361)
    }
}