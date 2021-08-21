#importing library
import random
import getpass
#login function
def passw():
    r=getpass.getpass("password : ")
    rp='4883'
    if r==rp:
        pass
    else:
        quit()
#variables
a=0
cont='y'
#dataset
li=['mag','tuver','masoor','thebra','thepla/masala bhakri','paratha','vinu bhai khichdi','tuver ma dhokdi','chora','pav bhaji','chole','rajma','vaal','turiya patra','bhagat muthiya','vaal ni daard','daad bhaat','masala khichdi','khada masala khichdi','frankie','burito','chatpata aloo naan','ragda petis','idli sambhar','dosa','ringad papdi','dana ringad','dhokla','daad ma dhokdi','dudhi chana','paneer angara','makkai nu saak']
#login
passw()
#UI
while cont=='y' or cont=='Y':
    print("1. one food item")
    print("2. multiple food items")
    print()
    ch=int(input("enter choice : "))
    if ch==1:
        print(random.choice(li))
        cont=input("do you want to continue? y/n ")
    elif ch==2:
        a=int(input("how many food choices? "))
        x=random.sample(li,a)
        print()
        for i in x:
            print(i)
        cont=input("do you want to continue? y/n ")


### add weights! to each food item as to how healthy it is and healthier items have a higher chance of being shown
