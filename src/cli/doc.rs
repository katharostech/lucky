//! Handles printing doc pages for both the commandline pager and the mdbook site

pub(crate) mod cmdln_pager;
pub(crate) mod mdbook;

// /// WIP: Print CLI doc structure to commandline
// pub fn print_full_cli_doc() {
//     let command = Box::new(crate::cli::LuckyCli);

//     fn print_doc(command: &dyn CliCommand, depth: usize) {
//         println!("{}Command Name: {}", " ".repeat(depth*4), command.get_name());
//         println!("{}Has doc: {}", " ".repeat(depth*4), match command.get_doc() {
//             Some(_) => "yes",
//             None => "no",
//         });

//         for subcommand in command.get_subcommands() {
//             print_doc(subcommand, depth+1);
//         }
//     }

//     print_doc(command, 0);
// }
