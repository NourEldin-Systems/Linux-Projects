# contacts = ["ahmed", "mahmoud", "ali"]
# n1 = len(contacts[0] + contacts[1] + contacts[2])
# n2 = len(contacts[0]) + len(contacts[1]) + len(contacts[2])

# print(n1)
# print(n2)

# print ("this is loop")

# if contacts[0] == "ahmed":
#     print("hello ahmed")

# if "" != 69:
#     print('they are equal')

# age = 18

# if age <= 18:
#     print ("older than 18")

# this is comment


user_type = "admin"
auth = False

if user_type == "admin":
    if auth:
        print("authorized")
    else:
        print("unauthorized")
else:
    print("Get out of here") 

if user_type =="admin" and auth:
    print("authorized")
elif user_type =="admin" and auth == False:
    print("unauthorized")
else:
    print("get out")
