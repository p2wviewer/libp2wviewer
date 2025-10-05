#!/bin/bash
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
N='\033[0m'
exec() {
    local num=$1
    local des=$2
    local cmd=$3
    
    echo -e "${BLUE}==== Test ${num}: ${des} ====${N}"
    echo -e "${YELLOW}Executing: ${cmd}${N}"
    if ! eval "$cmd"; then
        echo -e "${RED}Test ${num} Failed.${N}"
        exit 1
    else
        echo -e "${GREEN}Test ${num} passed.${N}"
    fi
}
rm -rf output
mkdir -p output
for i in {1..10}; do
    mkdir -p "output/test$i"
done
mkdir -p "output/errors"
exe=""
if [ -f "./libp2wviewer.exe" ]; then
    exe="./libp2wviewer.exe"
elif [ -f "./libp2wviewer" ]; then
    exe="./libp2wviewer"
else
    for dir in target/debug target/release; do
        if [ -f "$dir/libp2wviewer.exe" ]; then
            exe="$dir/libp2wviewer.exe"
            break
        elif [ -f "$dir/libp2wviewer" ]; then
            exe="$dir/libp2wviewer"
            break
        fi
    done
fi

if [ -z "$exe" ]; then
    echo "Error: Executable not found. The github Action should fail."
    exit 1
fi
cp input/test.png output/test1/test.png
exec 1 "Basic split into 2 parts" \
    "$exe split -i output/test1/test.png -n 2 -vvv"

cp input/test.png output/test2/test.png
exec 2 "Split into 3 parts with deletion" \
    "$exe split -i output/test2/test.png -n 3 -d -vvv"

cp output/test1/test.1.png output/test3/
cp output/test1/test.2.png output/test3/
exec 3 "Merge 2 parts" \
    "$exe merge --inputs output/test3/test.1.png,output/test3/test.2.png --output output/test3/merged.png -vvv"

cp output/test2/test.1.png output/test4/
cp output/test2/test.2.png output/test4/
cp output/test2/test.3.png output/test4/
exec 4 "Merge 3 parts" \
    "$exe merge --inputs output/test4/test.1.png,output/test4/test.2.png,output/test4/test.3.png --output output/test4/merged3.png -vvv"

cp input/test.png output/test5/test.png
exec 5 "Basic encryption with password" \
    "$exe encrypt -i output/test5/test.png -o output/test5/encrypted.png -p \"p2w\" -vvv"

cp input/test.png output/test6/test.png
cp input/keyfile output/test6/keyfile
exec 6 "Encryption with key file" \
    "$exe encrypt -i output/test6/test.png -o output/test6/encrypted_file.png --password-file output/test6/keyfile -vvv"

cp input/test.png output/test7/test.png
exec 7 "Encryption with splitting" \
    "$exe encrypt -i output/test7/test.png -o output/test7/encrypted_split.png -p \"p2w\" -s 3 -vvv"

cp output/test5/encrypted.png output/test8/
exec 8 "Basic decryption with password" \
    "$exe decrypt -i output/test8/encrypted.png -o output/test8/decrypted.png -p \"p2w\" -vvv"

cp output/test6/encrypted_file.png output/test9/
cp input/keyfile output/test9/keyfile
exec 9 "Decryption with key file" \
    "$exe decrypt -i output/test9/encrypted_file.png -o output/test9/decrypted_file.png --password-file output/test9/keyfile -vvv"

cp output/test7/encrypted_split.1.png output/test10/
cp output/test7/encrypted_split.2.png output/test10/
cp output/test7/encrypted_split.3.png output/test10/
exec 10 "Decryption of split encrypted file" \
    "$exe decrypt -i output/test10 -o output/test10/decrypted_split.png -p \"p2w\" -vvv"

cp input/test.png output/errors/test.png
cp output/test5/encrypted.png output/errors/

echo "==== Testing Errors ===="
echo "Testing nonexistent input file..."
if $exe split -i nonexistent.png -n 2 -vvv; then
    echo "Error case failed: Should have errored on nonexistent file"
    exit 1
fi
echo "Testing merge with nonexistent parts..."
if $exe merge --inputs missing1.png,missing2.png --output output/errors/bad.png -vvv; then
    echo "Error case failed: Should have errored on missing input files"
    exit 1
fi
echo "Testing encryption without password..."
if $exe encrypt -i output/errors/test.png -o output/errors/fail.png -vvv; then
    echo "Error case failed: Should have errored without password"
    exit 1
fi
echo "Testing decryption with wrong password..."
if $exe decrypt -i output/errors/encrypted.png -o output/errors/fail.png -p "f2p" -vvv; then
    echo "Error case failed: Should have errored with wrong password"
    exit 1
fi
echo -e "${GREEN}All tests passed.${N}"
rm -rf output