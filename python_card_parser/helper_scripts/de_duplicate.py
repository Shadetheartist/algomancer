def unique_strings(strings):
    seen = set()
    unique = []
    for string in strings:
        if string not in seen:
            unique.append(string)
            seen.add(string)
    return unique


# Example usage:
input_strings = [
    '''-1/-1 counter on each {/n}unit''',
    '''-1/-1 counter on me. if you do''',
    '''counter on an enemy''',
    '''-1/-1 counter on each {/n}unit''',
    '''+1/+1 counter on each of your units. {/n}[augment] when i despawn''',
    '''-1/-1 counter on each {/n}unit''',
    '''counter on an enemy''',

]
result = unique_strings(input_strings)
for s in result:
    s = s.replace('{/n}', '')
    s = s.replace('\'', '\\\'')
    print("| " + '\'' + s + '\'')
