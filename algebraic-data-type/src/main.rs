use std::marker::PhantomData;
use std::ops::Add;
use std::ptr::addr_eq;
use anyhow::{anyhow};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
enum OrderStatus {
    Pending {
        started_at: DateTime<Utc>,
    },
    Paid {
        paid_at: DateTime<Utc>,
        txn_id: Option<String>,
    },
    Shipped {
        shipped_at: DateTime<Utc>,
        tracking_number: String,
        delivery_provider: String,
    },
    Cancelled {
        cancelled_at: DateTime<Utc>,
        reason: String,
        refunded: bool,
    },
}


#[derive(Debug, Clone)]
enum PaymentMethod {
    BankTransfer {
        payee_bank_account: String,
    },
    CashOnDelivery {},
    CreditCard {
        card_brand: CardBrand,
    },
}

impl PaymentMethod {
    fn code(&self) -> String {
        match self {
            PaymentMethod::BankTransfer { .. } => "B".to_owned(),
            PaymentMethod::CashOnDelivery {} => "COD".to_owned(),
            PaymentMethod::CreditCard { .. } => "CR".to_owned(),
        }
    }
    fn deadline_rule(&self) -> Option<PaymentDeadlineRule> {
        match self {
            PaymentMethod::BankTransfer{ .. } => {
                Some(PaymentDeadlineRule {
                    deadline_dates: Duration::days(20)
                })
            }
            PaymentMethod::CashOnDelivery{} => {
                None
            }
            PaymentMethod::CreditCard{ .. } => {
                Some(PaymentDeadlineRule {
                    deadline_dates: Duration::days(15)
                })
            }
        }
    }
}

struct PaymentDeadlineRule {
    deadline_dates: Duration,
}

#[derive(Debug, Clone, PartialEq)]
enum CardBrand {
    Visa,
    JCB,
    Master,
    Amex,
    Diners,
}

#[derive(Debug, Clone, PartialEq)]
struct OrderId(String);

#[derive(Debug, Clone)]
struct Order {
    id: OrderId,
    payment_method: PaymentMethod,
    status: OrderStatus,
}

impl Order {
    fn new(ordered_at: DateTime<Utc>) -> Self {
        Order {
            id: OrderId("order-0000000001".to_string()),
            payment_method: PaymentMethod::BankTransfer {
                payee_bank_account: "三井住友銀行".to_owned()
            },
            status: OrderStatus::Pending {
                started_at: ordered_at,
            },
        }
    }

    fn capture(&mut self, now: DateTime<Utc>, txn_id: Option<String>) -> Result<(), anyhow::Error> {
        match self.status {
            OrderStatus::Pending { started_at, .. } => {
                if let Some(rule) = self.payment_method.deadline_rule() {
                    if now - started_at > rule.deadline_dates {
                        return Err(anyhow!("capture deadline exceeded. should be capture in {} days", rule.deadline_dates.num_days()));
                    };
                }

                self.status = OrderStatus::Paid {
                    paid_at: now,
                    txn_id,
                };
                Ok(())
            }
            _ => Err(anyhow!("invalid operation"))
        }
    }
}

struct Unverified;
struct Verified;
struct Blocked;

struct Email<State> {
    address: String,
    _state: PhantomData<State>
}

impl Email<Unverified> {
    fn new(address: String) -> Result<Self, String> {
        if address.contains('@') {
            Ok(Email{
                address,
                _state: PhantomData
            })
        } else {
            Err("invalid format".to_string())
        }
    }

    fn verify(self, code: &str) -> Result<Email<Verified>, String> {
        if code == "123456" {
            Ok(Email{
                address: self.address,
                _state: PhantomData,
            })
        } else {
            Err("invalid varification code".to_string())
        }
    }
}

impl Email<Verified> {
    fn block(&self) -> Email<Blocked> {
        Email{
            address: self.address.to_string(),
            _state: PhantomData,
        }
    }
}
impl <State> Email<State> {
    fn as_str(&self) -> &str {
        return self.address.as_str()
    }
    fn into(&self) -> String {
        return self.address.clone()
    }
}
fn main() {
    {
        let today = Utc::now();
        let ordered_at = today.add(Duration::days(19) * -1);
        println!("ordered_at: {:?}, today: {:?}", ordered_at, today);

        let mut order = Order::new(ordered_at);
        let result = order.capture(today, None);
        println!("{:?}", result);
    }

    {
        let today = Utc::now();
        let ordered_at = today.add((Duration::days(21) * -1));
        println!("ordered_at: {:?}, today: {:?}", ordered_at, today);

        let mut order = Order::new(ordered_at);
        let result = order.capture(today, None);
        println!("{}", result.err().unwrap());
    }

    let email = Email::new("info@example.com".to_string()).unwrap();
    let verify = email.verify("123456").unwrap();
    println!("{:?}", verify.as_str());
}