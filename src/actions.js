
export const setInputText = (text) => {
  return {
    type: 'SET_INPUT_TEXT',
    text
  }
};

export const pushStack = (text) => {
  return {
    type: 'PUSH_STACK',
    text
  }
};

export const executeOperator = (op) => {
  return {
    type: 'EXECUTE_OPERATOR',
    op
  }
};

export const appendInput = (text) => {
  return {
    type: 'APPEND_INPUT',
    text
  }
};