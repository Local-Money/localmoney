function sleep(ms) {
  return new Promise((res) => setTimeout(res, ms));
}

export { sleep };
