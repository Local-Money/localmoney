import firebase from "firebase/compat";
import { doc, setDoc } from 'firebase/firestore'
import firebaseConfig from "./firebaseConfig";

const config = firebaseConfig
const firebaseApp = firebase.initializeApp(config)

export const db = firebaseApp.firestore()
export const tradesCollection = db.collection('trades')

export const updateTrade = async (trade) => {
  const tradeRef = doc(db, 'trades', trade.addr)
  await setDoc(tradeRef, trade, { merge: true })
}

export const newTrade = async (maker, newTradeMsg) => {
  const tradeRequestRef = doc(db, 'tradeRequests', maker)
  await setDoc(tradeRequestRef, newTradeMsg, { merge: true })
}
