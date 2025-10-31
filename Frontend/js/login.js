


document.addEventListener("DOMContentLoaded",()=>{

    const form = document.getElementById('loginForm');
    
    document.getElementById("loginForm").addEventListener("submit",(event)=>{
        
        event.preventDefault()
        

        const formData = new FormData(event.target);
        const name = formData.get('password');
        let submittedForm = Object.fromEntries(formData.entries())
        form.reset();
        
        console.log(submittedForm)


        // let login = fetch("",(response)=>{})
     



    })
    





})