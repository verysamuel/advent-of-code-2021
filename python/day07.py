from statistics import median, mean
from math import floor

input = list(map(int,open("../input/day07/input.txt").read().split(",")))

def calculate_fuel_cost(midpoint, fuel_cost):
    return sum(map(lambda crab: fuel_cost(abs(crab - midpoint)), input))

def triangle_numbers(distance):
    return (distance * (distance + 1)) / 2

print("Part 1:", calculate_fuel_cost(median(input), lambda x: x))
print("Part 2:", calculate_fuel_cost(floor(mean(input)), triangle_numbers))