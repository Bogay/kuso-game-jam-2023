disc = {
    "StoneTool": "StoneTool",
    "BronzeTool": "BronzeTool",
    "IronTool": "IronTool",
    "SteelTool": "SteelTool",
    "SteamPower": "SteamPower",
    "ElectronicTechnology": "ElectronicTechnology",
    "Religion": "Religion",
    "Chiefdom": "Chiefdom",
    "Feudal": "Feudal",
    "Centralization": "Centralization",
    "Democracy": "Democracy",
    "Theocracy": "Theocracy",
    "Monarchy": "Monarchy",
    "Empire": "Empire",
    "Totalitarian": "Totalitarian",
    "PermanentMember": "PermanentMember",
    "Wheat": "Wheat",
    "Alcohol": "Alcohol",
    "Meat": "Meat",
    "Fish": "Fish",
    "GatheringAndHunting": "GatheringAndHunting",
    "Fishery": "Fishery",
    "Writing": "Writing",          
    "Book": "Book",
    "Printing": "Printing",
    "Currency": "Currency",
    "Trading": "Trading",
    "Industrialization": "Industrialization",
}

while 1:
    s = input()
    id = s[4:-1]
    # print(f'''"{id}": "{id}",''')
    print(f'''
        (((1, 1)), (
            id: {id},
            name: "{id}",
            description: "{disc[id]}",
            texture_id: {id},
        )),''',end='')