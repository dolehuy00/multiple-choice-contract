//random elements location in arrray
function shuffleArray(array) {
  var ctr = array.length,
    temp,
    index;
  // While there are elements in the array
  while (ctr > 0) {
    // Pick a random index
    index = Math.floor(Math.random() * ctr);
    // Decrease ctr by 1
    ctr--;
    // And swap the last element with it
    temp = array[ctr];
    array[ctr] = array[index];
    array[index] = temp;
  }
  return array;
}

//random answer and question
function randomQA(questionData) {
  const array = [];
  questionData.forEach((item, index) => {
    item.answers = shuffleArray(item.answers);
    array.push(item);
  });
  return shuffleArray(array);
}

//init state
State.init({
  answers: [], // Mảng để lưu trữ câu trả lời của người dùng cho mỗi câu hỏi
  showOptions: false,
  isCorrect: null, // Biến để kiểm tra câu trả lời
  questionData: randomQA(
    Near.view("dev-1692944401263-67806333947747", "get_questions")
  ),
});

//link contract https://github.com/dolehuy00/multiple-choice-contract.git

//near call dev-1692944401263-67806333947747 create_question '{"question_id": "idString", "title": "Question?", "answers":{"A": "AnswerA", "B": "AnswerB", "C": "AnswerC", "D": "AnswerD"}, "correct_abc": "D"}' --account-id xxxx.testnet

// Get index correct answer
function arrayCorrect(arrayQuestion) {
  const arrayCorrect = [];
  arrayQuestion.forEach((itemQ) => {
    itemQ.answers.forEach((itemA, indexA) => {
      if (itemQ.correct === itemA) {
        arrayCorrect.push(indexA);
      }
    });
  });
  return arrayCorrect;
}

//change index to abcd
function changeIndexToAbc(index) {
  const alphabetValues = ["A", "B", "C", "D"];
  return alphabetValues[index];
}

//compare two array
function isCompareArray(arr1, arr2) {
  return JSON.stringify(arr1) === JSON.stringify(arr2);
}

//onChange radiobutton
const handleAnswer = (questionIndex, selectedAnswer) => {
  // Cập nhật câu trả lời đã chọn cho câu hỏi được chỉ định
  const updatedAnswers = [...state.answers];
  updatedAnswers[questionIndex] = selectedAnswer;

  // Kiểm tra xem cả hai câu trả lời đã được chọn
  const allAnswersSelected =
    updatedAnswers.length === state.questionData.length;

  State.update({
    answers: updatedAnswers,
    showOptions: allAnswersSelected,
    isCorrect: null, // Đặt lại biến kiểm tra khi người dùng chọn câu trả lời mới
  });
};

//submit button check
const handleCheck = () => {
  // Kiểm tra câu trả lời và đưa ra thông báo
  if (isCompareArray(arrayCorrect(state.questionData), state.answers)) {
    State.update({
      isCorrect: true,
    });
  } else {
    State.update({
      isCorrect: false,
    });
  }
};
//submit button refresh
const handleRefresh = () => {
  State.update({
    answers: [],
    showOptions: false,
    isCorrect: null,
    questionData: randomQA(state.questionData),
  });
};

const radioStyle = {
  borderRadius: "50%", // Làm tròn góc nút radio
  border: "2px solid #333",
  backgroundColor: "#fff",
  width: "20px", // Chiều rộng của nút radio
  height: "20px", // Chiều cao của nút radio
  position: "absolute",
  left: "0",
  top: "2px",
};

const checkedRadioStyle = {
  backgroundColor: "#333",
};

return (
  <div>
    <br />
    <hr />
    <div class="questions">
      {state.questionData.map((itemQ, indexQ) => (
        <div>
          <h5>
            Câu hỏi {indexQ + 1}: {itemQ.question}
          </h5>
          {state.questionData[indexQ].answers.map((itemA, indexA) => (
            <div style={{ position: "relative" }}>
              <input
                class=""
                type="radio"
                id={"answer" + (indexQ + 1) + indexA}
                name={"question" + (indexQ + 1)}
                value={indexA}
                onChange={() => handleAnswer(indexQ, indexA)}
                style={{ ...radioStyle }}
                checked={state.answers[indexQ] === indexA}
              />
              <label
                htmlFor={"answer" + (indexQ + 1) + indexA}
                style={{ marginLeft: "40px" }}
              >
                {changeIndexToAbc(indexA) + ". " + itemA}
              </label>
            </div>
          ))}
          <br />
        </div>
      ))}
    </div>

    {state.showOptions && (
      <div>
        <button onClick={handleCheck}>Trắc Nghiệm</button>
      </div>
    )}
    {state.isCorrect !== null && (
      <div>
        {state.isCorrect ? (
          <p>Bạn đã chính xác!</p>
        ) : (
          <p>Bạn chưa chính xác.</p>
        )}
        <button onClick={handleRefresh}> Tai lai bo cau hoi</button>
      </div>
    )}
    <br />
  </div>
);
