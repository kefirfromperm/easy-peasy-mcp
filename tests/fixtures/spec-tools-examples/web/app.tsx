// SPDX-License-Identifier: MIT
// any props are passed on to the page
import { Page } from "./page";

export function App(props: any) {
  const company = props.company;
  return <Page title={company} />;
}
