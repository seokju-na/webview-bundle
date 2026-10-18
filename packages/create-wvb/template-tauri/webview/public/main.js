let count = 0;
const button = document.querySelector('#counter');
button.addEventListener('click', () => {
  button.textContent = `Count: ${++count}`;
});
