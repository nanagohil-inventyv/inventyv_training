const NUMS = Symbol("nums");
const NUMS2 = Symbol("nums2");
const Submit = document.getElementById("button")
let  myPromise;
Submit.onclick = firstFun
function firstFun() {
    const arrObj = {
        [NUMS]: [1, 2, 3, 3, 4, 5, 6, 7, 8]
    }
    const firstElement = arrObj[NUMS].shift();
    myPromise = SecondFun(firstElement, arrObj);
    myPromise.then((value) => {
        console.log(value);
    }).catch((value) => {
        console.log(value);
    })
}

function SecondFun(firstElement, arrObj) {
    const arrObj2 = {
        [NUMS2]: [9, 10, 11, 12]
    }
    arrObj2[NUMS2].unshift(firstElement);
    arrObj2[NUMS2].push(...arrObj[NUMS])
    const promiseObj = new Promise(function (resolve, reject) {
        const totalSum = arrObj2[NUMS2].reduce((acc, currValue) => acc + currValue, 0);
        if (totalSum > 35) {
            resolve(`The Promise is Resolved and totalSum = ${totalSum}`);
        } else {
            reject(`The Promise is Rejected and totalSum = ${totalSum}`);
        }

    })
    return promiseObj;
}