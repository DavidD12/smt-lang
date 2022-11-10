// https://en.wikipedia.org/wiki/Zebra_Puzzle

struct Color {}
inst Yellow, Blue, Red, Ivory, Green: Color

struct Nationality {}
inst Norwegian, Ukrainian, Englishman, Spaniard, Japanese: Nationality

struct Drink {}
inst Water, Tea, Milk, Orange_juice, Coffee: Drink

struct Smoke {}
inst Kools, Chesterfield, Old_Gold, Lucky_Strike, Parliament: Smoke

struct Pet {}
inst Fox, Horse, Snails, Dog, Zebra: Pet

struct House {
    color: Color
    nationality: Nationality
    drink: Drink
    smoke: Smoke
    pet: Pet
    left: House
    right: House
}

inst House_1, House_2, House_3, House_4, House_5: House

/*
constraint opposite = (
    (forall house: House | house.color.house = house) and
    (forall house: House | house.nationality.house = house) and
    (forall house: House | house.drink.house = house) and
    (forall house: House | house.smoke.house = house) and
    (forall house: House | house.pet.house = house)
)

// useless with opposite !
constraint alldiff = (
    (forall x, h: House | x.color = y.color => x = y) and
    (forall x, h: House | x.nationality = y.nationality => x = y) and
    (forall x, h: House | x.drink = y.drink => x = y) and
    (forall x, h: House | x.smoke = y.smoke => x = y) and
    (forall x, h: House | x.pet = y.pet => x = y)
)
*/

constraint left = (
    House_1.left = House_5 and 
    House_2.left = House_1 and
    House_3.left = House_2 and
    House_4.left = House_3 and
    House_5.left = House_4
)

constraint right = (
    House_1.right = House_2 and
    House_2.right = House_3 and
    House_3.right = House_4 and
    House_4.right = House_5 and
    House_5.right = House_1
)

let green_house: House
constraint green = (green_house.color = Green)

let englishman_house: House
constraint englishman = (englishman_house.nationality = Englishman)

let spaniard_house: House
constraint spaniard = (spaniard_house.nationality = Spaniard)

let ukrainian_house: House
constraint ukrainian = (ukrainian_house.nationality = Ukrainian)

let old_gold_house: House
constraint old_gold = (old_gold_house.smoke = Old_Gold)

let yellow_house: House
constraint yellow = (yellow_house.color = Yellow)

let chesterfield_house: House
constraint chesterfield = (chesterfield_house.smoke = Chesterfield)

let kools_house: House
constraint kools = (kools_house.smoke = Kools)

let lucky_strike_house: House
constraint lucky_strike = (lucky_strike_house.smoke = Lucky_Strike)

let japanese_house: House
constraint japanese = (japanese_house.nationality = Japanese)

let norwegian_house: House
constraint norwegian = (norwegian_house.nationality = Norwegian)

constraint alldiff = (
    House_1.drink /= House_2.drink
    // TODO
)

constraint R02 = (englishman_house.color = Red)
constraint R03 = (spaniard_house.pet = Dog)
constraint R04 = (green_house.drink = Coffee)
constraint R05 = (ukrainian_house.drink = Tea)
constraint R06 = (green_house.left.color = Ivory and green_house /= House_1)
constraint R07 = (old_gold_house.pet = Snails)
constraint R08 = (yellow_house.smoke = Kools)
constraint R09 = (House_3.drink = Milk)
constraint R10 = (House_1.nationality = Norwegian)
constraint R11 = (
    (chesterfield_house.left.pet = Fox and chesterfield_house /= House_1) 
    or
    (chesterfield_house.right.pet = Fox and chesterfield_house /= House_5) 
)
constraint R12 = (
    (kools_house.left.pet = Horse and kools_house /= House_1)
    or
    (kools_house.right.pet = Horse and kools_house /= House_5)
)
constraint R13 = (lucky_strike_house.drink = Orange_juice)
constraint R14 = (japanese_house.smoke = Parliament)
constraint R15 = (
    (norwegian_house.left.color = Blue and norwegian_house /= House_1) 
    or 
    (norwegian_house.right.color = Blue and norwegian_house /= House_5)
)