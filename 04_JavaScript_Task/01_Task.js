
const Submit = document.getElementById("button")
Submit.onclick = firstFun
let  myPromise;
function firstFun() {
    const nums = [1, 2, 3, 3, 4, 5, 6, 7, 8];
    const firstElement = nums.shift();
    myPromise = SecondFun(firstElement, nums);
    myPromise.then((value) => {
        console.log(value);
    }).catch((value) => {
        console.log(value);
    })
}

function SecondFun(firstElement, nums) {
    const nums2 = [9, 10, 11, 12];
    nums2.unshift(firstElement);
    nums2.push(...nums)
    const promiseObj = new Promise(function (resolve, reject) {
        const totalSum = nums2.reduce((acc, currValue) => acc + currValue, 0);
        if (totalSum > 35) {
            resolve(`The Promise is Resolved and totalSum = ${totalSum}`);
        } else {
            reject(`The Promise is Rejected and totalSum = ${totalSum}`);
        }

    })
    return promiseObj;
}