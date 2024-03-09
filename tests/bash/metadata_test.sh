TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENTRY_FILE_PATH="${TESTING_DIR}/fixtures/entry.md"
METADATA=''

# TODO: move this to a test utils file
createAndGetTmpEntry() {
  local tmpMetaFile=$(mktemp /tmp/videas.XXXXXX)
  cp "$ENTRY_FILE_PATH" "$tmpMetaFile"
  echo "$tmpMetaFile"
}


testExtractMetadataBlock() {
  local tmpFile=$(createAndGetTmpEntry)
  local result=$(extractMetadataBlock "$tmpFile")
 
  assertNotNull "$result"
  assertNotContains "$(< $tmpFile)" "===="
}

testExtractTitle() {
  local result=$(extractTitle "$METADATA")
  local expected="This is an interesting article"

  assertNotNull "$result"
  assertEquals "$expected" "$result"
}

testExtractSlug() {
  local result=$(extractSlug "$METADATA")
  local expected="interesting-article"

  assertNotNull "$result"
  assertEquals "$expected" "$result"
}

testExtractTags() {
  local result=$(extractTags "$METADATA")

  assertNotNull "$result"
  assertContains "$result" "Tag1"
  assertNotContains "$result" "Tag10"
}

oneTimeSetUp(){
  SHUNIT_COLOR='always'
  source "${TESTING_DIR}/../../bash/metadata.sh"

  local tmpFile=$(createAndGetTmpEntry)
  METADATA=$(extractMetadataBlock "$tmpFile")
}

source "${TESTING_DIR}/shunit2/shunit2"
