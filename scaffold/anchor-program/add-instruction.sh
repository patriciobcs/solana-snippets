#!/bin/bash;

instruction="generic_instruction"

if ! [ -z ${add+x} ]; then instruction=$add; fi

mkdir -p src/instructions
touch src/instructions/mod.rs
instruction_file=src/instructions/$instruction.rs
touch $instruction_file
instruction_camel_case=$(echo ${instruction//_/ } | awk '{for(i=1;i<=NF;i++) {printf("%s%s",toupper(substr($i,1,1)),substr($i,2))} }')

echo "\npub mod $instruction;\npub use $instruction::*;" >> src/instructions/mod.rs

echo "use anchor_lang::prelude::*;\n\n#[derive(Accounts)]\npub struct $instruction_camel_case {}\n\npub fn processor(_ctx: Context<$instruction_camel_case>) -> Result<()> {\n\n\t\tOk(())\n}" > $instruction_file

lib=$(sed '/}$/r'<(
    echo "    pub fn $instruction(ctx: Context<$instruction_camel_case>) -> Result<()> {";
    echo "        instructions::$instruction::processor(ctx)?;";
    echo "        Ok(())";
    echo "    }";
) src/lib.rs)

echo "$lib" > src/lib.rs
