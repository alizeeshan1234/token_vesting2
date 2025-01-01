export const IDL = {
    "version": "0.1.0",
    "name": "token_vesting",
    "constants": [
        {
            "name": "ANCHOR_DISCREMENATOR",
            "type": {
                "defined": "usize"
            },
            "value": "8"
        }
    ],
    "instructions": [
        {
            "name": "initilaizeAccount",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "tokenVestingAccount",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "tokenVaultAccount",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "mint",
                    "isMut": false,
                    "isSigner": false
                },
                {
                    "name": "systemProgram",
                    "isMut": false,
                    "isSigner": false
                },
                {
                    "name": "tokenProgram",
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "companyName",
                    "type": "string"
                },
                {
                    "name": "vestingAmount",
                    "type": "u64"
                },
                {
                    "name": "vestingStartTime",
                    "type": "i64"
                },
                {
                    "name": "vestingEndTime",
                    "type": "i64"
                },
                {
                    "name": "vestingCliffPeriod",
                    "type": "i64"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "tokenVestingData",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "companyName",
                        "type": "string"
                    },
                    {
                        "name": "totalVestingAmount",
                        "type": "u64"
                    },
                    {
                        "name": "totalClaimedAmount",
                        "type": "u64"
                    },
                    {
                        "name": "startTime",
                        "type": "i64"
                    },
                    {
                        "name": "endTime",
                        "type": "i64"
                    },
                    {
                        "name": "cliffPeriod",
                        "type": "i64"
                    }
                ]
            }
        }
    ],
    "types": [
        {
            "name": "Error",
            "type": {
                "kind": "enum",
                "variants": [
                    {
                        "name": "InvalidPublicKey"
                    },
                    {
                        "name": "InvalidVestingTime"
                    },
                    {
                        "name": "InvalidVestingAmount"
                    },
                    {
                        "name": "InvalidCliffPeriod"
                    }
                ]
            }
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "CustomError",
            "msg": "Custom error message"
        }
    ]
};
