#!/usr/bin/python

def is_increasing(perm):
	print perm
	if (len(perm) > 1):
		for idx in range(len(perm)-1):
			if (perm[idx] > perm[idx+1]):
				return False
	return True

def play_game(play_list, player=0):
	if len(play_list) <= 2:
		return player
	else:
		for next_move in range(len(play_list)):
			if (is_increasing(
			    play_list[0:next_move] + 
			    play_list[next_move+1:])):
				return player
		for next_move in range(len(play_list)):
			opp_list = play_list[0:next_move] + play_list[next_move+1:]
			if (is_optimal(opp_list)):
				print '---'
				return play_game(opp_list, player ^ 1)
			print '-'
		return (player ^ 1)


def is_optimal(play_list):
	for opp_move in range(len(play_list)):
		if (is_increasing(
		    play_list[0:opp_move] +
		    play_list[opp_move+1:])):
			return False
	return True
	print 'No Winner'
	return player


num_cases = int(raw_input())
perm_num = []
perm_list = []
for test_case in range(num_cases):
	perm_num.append(int(raw_input()))
	perm_list.append([int(num) for num in raw_input().split()])

print perm_list

print is_increasing(perm_list[0])

for test_case in range(num_cases):
	winner = "Alice" if play_game(perm_list[test_case]) == 0 else "Bob"
	print(winner)
