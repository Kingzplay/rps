use rock_paper_scissor::*; //imports all the items present in the lib.rs file

fn main() {
    println!("Rock,paper,scissor game. Type \"help\" to display the commands.\nThis program isn't case sensitive.");

    loop {
        let prompt = uis::uis(">").to_lowercase();   
        match prompt.as_str() {
            "help" => println!("Commands:\nHelp:\tDisplays valid commands and their utility.\nRock:\tSets the Rock element as your choice\nPaper:\tSets the Paper element as your choice\nScissor: Sets the Scissor element as your choice\nExit:\tExits the program\nCredits: Displays credits\nℹ️\tThe commands aren't case sensitive."),
            "exit" => break, 
            "rock" => matcher(prompt),
            "paper" => matcher(prompt),
            "scissor" => matcher(prompt),
            "credits" => println!("Thanks for the Rust team for creating this amazing programming language\n"),
            _ => println!("Unknown command, type \"help\" to display valid commands.")
    }
    }
}
