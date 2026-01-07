
#!/usr/bin/env bash

# Usage:
# ./scripts/new_problem.sh 3512. Minimum Operations to Make Array Sum Divisible by K
# ./scripts/new_problem.sh LC-3512 Minimum Operations to Make Array Sum Divisible by K

set -e

RAW_NUM=$1
shift

if [[ -z "$RAW_NUM" || "$#" -eq 0 ]]; then
    echo "Usage: $0 <problem_number> <problem_title...>"
    exit 1
fi

# Extract digits only from problem number
NUM=$(echo "$RAW_NUM" | sed 's/[^0-9]//g')

if [[ -z "$NUM" ]]; then
    echo "❌ Invalid problem number: $RAW_NUM"
    exit 1
fi

# Remaining args form the title
TITLE="$*"

# Convert title to snake_case
SNAKE_TITLE=$(echo "$TITLE" \
        | tr '[:upper:]' '[:lower:]' \
        | sed 's/[^a-z0-9 ]//g' \
    | sed 's/ \+/_/g')

MODULE_NAME="p${NUM}_${SNAKE_TITLE}"
PROBLEM_DIR="src/problems/${MODULE_NAME}"

if [[ -d "$PROBLEM_DIR" ]]; then
    echo "❌ Problem already exists: $MODULE_NAME"
    exit 1
fi

mkdir -p "$PROBLEM_DIR"

# Create mod.rs template
cat > "$PROBLEM_DIR/mod.rs" <<EOF
// ${NUM}. ${TITLE}

struct Solution;

impl Solution {
    pub fn $SNAKE_TITLE() {
        todo!("Implement solution for ${NUM}. ${TITLE}");
    }
}

pub fn run() {
    println!("Running problem ${NUM}: ${TITLE}");
    println!("{:?}", Solution::$SNAKE_TITLE());
}
EOF

# Register module
echo "pub mod ${MODULE_NAME};" >> src/problems/mod.rs

cat > "src/main.rs" <<EOF
#[allow(dead_code)]
mod problems;
fn main() {
    problems::$MODULE_NAME::run();
}
EOF

echo "✅ Created problem: ${MODULE_NAME}"
