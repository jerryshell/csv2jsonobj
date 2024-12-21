# CSV to JSON Object

Well, it is what it sounds like.

## Install

```bash
cargo install --locked --git https://github.com/jerryshell/csv2jsonobj
```

## Usage

```bash
csv2jsonobj <csv_path> [key_index]
```

`key_index` is optional, default is 0

## Example

```bash
$ cat example.csv
id,name,equiment_slot,stackable,attack,defense,block
sword,Sword,MainHand,FALSE,60,0,0
shield,Shield,OffHand,FALSE,0,100,25
steel_helmet,Steel Helmet,Head,FALSE,0,80,0
leather_armor,Leather Armor,Chest,FALSE,0,110,0
jagged_arrow,Jagged Arrow,Projectiles,TRUE,20,0,0

$ csv2jsonobj example.csv > output.json

$ cat output.json
{
  "jagged_arrow": {
    "attack": 20.0,
    "block": 0.0,
    "defense": 0.0,
    "equiment_slot": "Projectiles",
    "id": "jagged_arrow",
    "name": "Jagged Arrow",
    "stackable": true
  },
  "leather_armor": {
    "attack": 0.0,
    "block": 0.0,
    "defense": 110.0,
    "equiment_slot": "Chest",
    "id": "leather_armor",
    "name": "Leather Armor",
    "stackable": false
  },
  "shield": {
    "attack": 0.0,
    "block": 25.0,
    "defense": 100.0,
    "equiment_slot": "OffHand",
    "id": "shield",
    "name": "Shield",
    "stackable": false
  },
  "steel_helmet": {
    "attack": 0.0,
    "block": 0.0,
    "defense": 80.0,
    "equiment_slot": "Head",
    "id": "steel_helmet",
    "name": "Steel Helmet",
    "stackable": false
  },
  "sword": {
    "attack": 60.0,
    "block": 0.0,
    "defense": 0.0,
    "equiment_slot": "MainHand",
    "id": "sword",
    "name": "Sword",
    "stackable": false
  }
}
```

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)
