# hanami
animal crossing new horizons flower breeding calculator

Calculates a breeding path from any set of genotype distributions to a specified phenotype.
The goal is to be able to ask "what do you have available, and what do you want?" and for 
hanami to be able to produce a sequence where breeding the flowers in that sequence and observing the correct phenotype output
will gurantee the desired flower result is found.

Unfortunately the algorithm isn't amazing. It does work but it doesn't give you the shortest path right now.

In the future I'd like to deploy this to a static web page in a web assembly package.

here is an example of how to breed a purple rose (with a specific genotype):

```json
{
    "color": "Purple",
    "expectedTime": 6.0,
    "genotypes": [
        "rryywwss",
        "rryYwwss"
    ],
    "parents": [
        {
            "color": "White",
            "expectedTime": 2.0,
            "genotypes": [
                "rryYWwss"
            ],
            "parents": [
                "white seed",
                "yellow seed"
            ]
        },
        "white seed"
    ]
}

            "color": "White",
