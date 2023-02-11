def find_floor(instructions):
	current_floor = []
	patterns = { "(": ")", ")": "(" }
	for pos, instruction in enumerate(instructions):
		if current_floor and current_floor[-1] == patterns[instruction]:
			current_floor.pop()
		else:
			if instruction == ")":
				return pos + 1 
			current_floor.append(instruction)
			
	return -1

with open("input") as f:
	lines = f.readlines()
	print(find_floor(lines[0].strip()))

