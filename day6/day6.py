from collections import deque, defaultdict

orbit_map = defaultdict(list)
# file_name = 'C:\\Users\\Carol\\Documents\\Projects\\aoc2019\\day6\\teste2.txt'
file_name = 'C:\\Users\\Carol\\Documents\\Projects\\aoc2019\\day6\\input.txt'

with open(file_name) as f:
    for line in f:
        orbited, orbiter = line.strip().split(')')
        orbit_map[orbited].append(orbiter)

planets_to_visit = deque([('COM', 0)])
visited_distances = []
ancestor_map = {'COM': frozenset()}

while planets_to_visit:
    planet, distance = planets_to_visit.popleft()

    if planet == 'YOU':
        you_distance = distance
    elif planet == 'SAN':
        san_distance = distance

    neighbour_planets = orbit_map[planet]

    planets_to_visit.extend([(neighbour, distance + 1) for neighbour in neighbour_planets])

    for neighbour in neighbour_planets:
        ancestor_map[neighbour] = ancestor_map[planet] | { (distance, planet) }

    visited_distances.append(distance)

print('Sum of distances is', sum(visited_distances))

you_ancestors = ancestor_map['YOU']
santa_ancestors = ancestor_map['SAN']

common_ancestors = you_ancestors & santa_ancestors

latest_common_ancestor = sorted(list(common_ancestors))[-1]

distance, _ = latest_common_ancestor

distance_you_to_san = (you_distance - distance) + (san_distance - distance) - 2

print('Distance YOU to SAN is', distance_you_to_san)