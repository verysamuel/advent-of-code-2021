input = open("../input/day02/input.txt", "r").readlines()

horizontal = 0
depth = 0
aim = 0
for line in input:
    (command, x) = line.split(" ")
    x = int(x)
    if command == "forward":
        horizontal += x
        depth += aim * x
    elif command == "up":
        # depth -= x
        aim -= x
    elif command == "down":
        # depth += x
        aim += x

print(depth * horizontal)