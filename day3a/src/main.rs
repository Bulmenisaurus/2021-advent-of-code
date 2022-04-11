use std::process::Command;

fn main() {
    let code = format!(
        "
input = '''{}'''.split('\\n')

report_length = len(input[0])
column_values = [[column[i] for column in input] for i in range(report_length)]

most_common_values = ['0' if i.count('0') > i.count('1') else '1' for i in column_values]
# could use a binary not but I couldn't figure out how it works
least_common_values = ['0' if i == '1' else '1' for i in most_common_values]

gamma_rate = int(''.join(most_common_values), 2)
epsilon_rate = int(''.join(least_common_values), 2)

print(gamma_rate * epsilon_rate)",
        include_str!("../input.txt")
    );

    let command = Command::new("python3")
        .args(["-c", &code])
        .output()
        .expect("expected this to work? idk");

    let output = String::from_utf8(command.stdout).unwrap();

    print!("{}", output);
}
