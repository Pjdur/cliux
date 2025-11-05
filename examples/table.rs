use cliux::Table;

fn main() {
    Table::new()
        .headers(&["Name", "Status"])
        .row(&["cliux", "active"])
        .row(&["other", "pending"])
        .widths(&[20, 10])
        .bordered(true)
        .print();
}
