// https://en.wikipedia.org/wiki/Zebra_Puzzle

class Elt {
    house: House
}

class Color extends Elt {}
inst Yellow, Blue, Red, Ivory, Green: Color

class Nationality extends Elt {}
inst Norwegian, Ukrainian, Englishman, Spaniard, Japanese: Nationality

class Drink extends Elt {}
inst Water, Tea, Milk, Orange_juice, Coffee: Drink

class Smoke extends Elt {}
inst Kools, Chesterfield, Old_Gold, Lucky_Strike, Parliament: Smoke

class Pet extends Elt {}
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

constraint opposite = (
    forall house: House | house.color.house = house end and
    forall house: House | house.nationality.house = house end and
    forall house: House | house.drink.house = house end and
    forall house: House | house.smoke.house = house end and
    forall house: House | house.pet.house = house end
)

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

constraint R02 = (Englishman.house.color = Red)
constraint R03 = (Spaniard.house.pet = Dog)
constraint R04 = (Green.house.drink = Coffee)
constraint R05 = (Ukrainian.house.drink = Tea)
constraint R06 = (Green.house.left.color = Ivory and Green.house /= House_1)
constraint R07 = (Old_Gold.house.pet = Snails)
constraint R08 = (Yellow.house.smoke = Kools)
constraint R09 = (House_3.drink = Milk)
constraint R10 = (House_1.nationality = Norwegian)
constraint R11 = (
    (Chesterfield.house.left.pet = Fox and Chesterfield.house /= House_1) 
    or
    (Chesterfield.house.right.pet = Fox and Chesterfield.house /= House_5) 
)
constraint R12 = (
    (Kools.house.left.pet = Horse and Kools.house /= House_1)
    or
    (Kools.house.right.pet = Horse and Kools.house /= House_5)
)
constraint R13 = (Lucky_Strike.house.drink = Orange_juice)
constraint R14 = (Japanese.house.smoke = Parliament)
constraint R15 = (
    (Norwegian.house.left.color = Blue and Norwegian.house /= House_1) 
    or 
    (Norwegian.house.right.color = Blue and Norwegian.house /= House_5)
)