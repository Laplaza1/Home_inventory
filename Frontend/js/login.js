


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
            
            login.then((response)=>response.json()).then((data)=>{console.log("data",data)})    
            })


        
    





})