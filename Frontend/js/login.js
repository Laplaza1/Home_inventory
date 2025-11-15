


document.addEventListener("DOMContentLoaded",()=>{

    const form = document.getElementById('loginForm');
    
    document.getElementById("loginForm").addEventListener("submit",(event)=>
        {
            document.getElementById("loading-screen").style.display="flex";document.getElementById("main-content").style.display="none";
            event.preventDefault()
            

            const formData = new FormData(event.target);
            const name = formData.get('password');
            let submittedForm = Object.fromEntries(formData.entries())
            form.reset();
            
            console.log(JSON.stringify(submittedForm))

            let login = fetch("https://home-inventory-bml1.onrender.com/login", 
            //let login = fetch("http://localhost:3000/login", 
                {
                    method:"POST",
                    headers: 
                        {
                            "Content-Type": "application/json"
                        },
                    credentials:"include",
                    body: JSON.stringify(submittedForm)
                })
            console.log(login)
            Promise.allSettled([Promise.resolve(login)]).then((result)=>
                {
                    console.log("Result ",result[0].value.status);
                    if (result[0].value.status!=200)
                        {
                            return alert(result[0].value.statusText)
                        }
                    result[0].value.json().then((data)=>
                        {
                            console.log(data.user_id)
                            let change = fetch("https://home-inventory-bml1.onrender.com/user",
                            //let change = fetch("http://localhost:3000/user", 
                                {
                                    method:"PUT",
                                    headers: 
                                        {
                                            "Content-Type": "application/json"
                                        },
                                    credentials:"include",
                                    body: JSON.stringify({"user_id":data.user_id})
                                }).then((evo)=>{console.log("all finished")})
                        })
                    window.location.replace(window.location.toString().substring(0,window.location.toString().lastIndexOf("/")+1)+"inventory.html");
                    
            
                })
              
            })


        
    





})