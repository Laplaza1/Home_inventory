


document.addEventListener("DOMContentLoaded",()=>{

    const form = document.getElementById('loginForm');
    
    document.getElementById("loginForm").addEventListener("submit",(event)=>
        {
        
            event.preventDefault()
            

            const formData = new FormData(event.target);
            const name = formData.get('password');
            let submittedForm = Object.fromEntries(formData.entries())
            form.reset();
            
            console.log(JSON.stringify(submittedForm))


            let login = fetch("https://home-inventory-bml1.onrender.com/login", 
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
                    
                    window.location.replace(window.location.toString().substring(0,window.location.toString().lastIndexOf("/")+1)+"inventory.html");
                    
            
                })
              
            })


        
    





})