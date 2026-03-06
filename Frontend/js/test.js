const socket = new WebSocket("ws://localhost:3000/web_socket")

socket.onopen = () => {
   console.log('Connected to server');
   socket.send('Hello Server!');
};
socket.onmessage = (event) => {
   console.log('Message from server:', event.data);
};
socket.onclose = () => {
   console.log('Disconnected from server');
};
socket.onerror = (error) => {
   console.error('WebSocket error:', error);
};