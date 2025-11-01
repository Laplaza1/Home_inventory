


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


            let login = fetch("http://localhost:3000/login", 
                {
                    method:"POST",
                    headers: 
                        {
                            "Content-Type": "application/json"
                        },
                    body: JSON.stringify(submittedForm)
                })
            Promise.allSettled([Promise.resolve(login)]).then((result)=>
                {
                    console.log("Result ",result[0].value.status);
                    if (result[0].value.status!=200)
                        {
                            return alert(result[0].value.statusText)
                        }
                    login.then((response)=>response.json()).then((data)=>
                        {
                            console.log("data",data)
                            cookieStore.set(
                                {
                                    "name":"Session_ID",
                                    "value":data.token,
                                    "expires":new Date(Date.now()+7 * 24 * 60 * 60 * 1000),
                                    "sameSite":"none"
                                })
                        })    
            
                })
              
            })


        
    





})