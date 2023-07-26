# Basic NF T  M etadata Server

Super basic, literally hardcoded serde JSON serialization for contract level and individual m etadata items. Nothing fancy. Docker build designed to be deployed and listen on port `4200`

## Routes

### `/contract-meta`

Return the Contract Metadata that OpenSea etc would want to see, corresponds to whatever your smart contract returns for contract level metadata URI

### `/meta/:tokenId`

Return the token specific (only ID 1 to 5) meta data for o pensea, including a super simple base64 encoded hardcoded image value that renders as SVG in opensea something something on-c hain data.
If you hit an ID outside of the specific range you'll get a 400 error, or maybe a different type will return a different HTTP status code from axum's typing

## Tests

Non-existent

## TODO

Cool n f t metadata functionality
